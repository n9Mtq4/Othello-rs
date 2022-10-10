import os
os.environ["CUDA_VISIBLE_DEVICES"] = "0"

import numpy as np
import torch
import torch.nn as nn
from torch.utils.tensorboard import SummaryWriter
from ranger21.ranger21 import Ranger21
from tqdm import tqdm

from othello_dataset import OthelloNegaDataset
from torch.utils.data import DataLoader


LR = 0.0005
# LR = 1e-5
EPOCHS = 2
BATCH_SIZE = 8192 // 2
NUM_WORKERS = 6
CSV_PATH = '/home/will/workspace/OthelloGui/train_data/novello6.4M_av_ns.npz'


class ResBlock(nn.Module):
    
    def __init__(self, f):
        super().__init__()
        self.l1 = nn.Linear(f, f)
        self.a1 = nn.ReLU()
        self.l2 = nn.Linear(f, f, bias=False)
        self.bn = nn.BatchNorm1d(f)
        self.a2 = nn.ReLU()
    
    def forward(self, x):
        resid = x
        x = self.l1(x)
        x = self.a1(x)
        x = self.l2(x)
        x = self.bn(x)
        x = self.a2(x + resid)
        return x


class OthelloNNetOld(nn.Module):
    
    def __init__(self, n, blocks):
        super().__init__()
        res_blocks = [ResBlock(n) for _ in range(blocks)]
        self.seq = nn.Sequential(
            nn.Linear(128, n),
            nn.ReLU(),
            *res_blocks,
            nn.Linear(n, 64),
            nn.ReLU(),
            nn.Linear(64, 1),
            nn.Tanh(),
        )
    
    def forward(self, x):
        x = self.seq(x)
        return x


class OthelloNNet(nn.Module):
    
    def __init__(self, n, blocks):
        super().__init__()
        self.en = nn.Sequential(
            nn.Linear(128, n),
            nn.ReLU(),
        )
        self.res_tower = nn.Sequential(*[ResBlock(n) for _ in range(blocks)])
        self.q_head = nn.Sequential(
            nn.Linear(n, 128),
            nn.ReLU(),
            nn.Linear(128, 1),
            nn.Tanh(),
        )
    
    def freeze_encoder(self):
        freeze = [self.en, self.res_tower]
        for module in freeze:
            for param in module.parameters():
                param.requires_grad = False
    
    def forward(self, x):
        x = self.en(x)
        x = self.res_tower(x)
        return self.q_head(x)


class ScaledMSELoss(nn.Module):
    
    def __init__(self):
        super().__init__()
    
    def forward(self, inputs, targets):
        mse_loss = (inputs - targets) ** 2
        weight = 2 * torch.exp(-torch.abs(10 * targets)) + 1
        return torch.mean(mse_loss * weight)


def main():
    
    print(f"{LR=}")
    print(f"{EPOCHS=}")
    print(f"{NUM_WORKERS=}")
    print(f"{CSV_PATH=}")
    
    print("Loading data...")
    dataset = OthelloNegaDataset(CSV_PATH)
    dataloader = DataLoader(
        dataset,
        batch_size=BATCH_SIZE,
        shuffle=True,
        num_workers=NUM_WORKERS,
        prefetch_factor=6,
        pin_memory=True,
    )
    
    model = OthelloNNet(1024, 8)
    
    model.load_state_dict(torch.load("chpt_50_adj_noq.pth")['model_state_dict'], strict=False)
    print("loaded model chpt")
    
    model = model.cuda()
    
    print(model)
    print(f"Total params = {sum(p.numel() for p in model.parameters())}")
    print(f"Trainable params = {sum(p.numel() for p in model.parameters() if p.requires_grad)}")
    
    print("Starting training...")
    train(model, dataloader)
    
    # print("Saving torchscript model...")
    # save_torchscript(model)


def train(model, dataloader):
    
    writer = SummaryWriter()
    
    criterion = ScaledMSELoss()
    
    optimizer = Ranger21(
        model.parameters(),
        lr=LR,
        num_epochs=EPOCHS,
        warmdown_start_pct=0.3,
        warmdown_min_lr=5e-7,
        num_batches_per_epoch=len(dataloader)
    )
    
    model.freeze_encoder()
    model.train()
    
    for epoch in range(EPOCHS):
        
        running_loss = []
        
        for step, (me, enemy, y) in tqdm(enumerate(dataloader), total=len(dataloader)):
            
            x = torch.cat((me, enemy), dim=1).float().cuda(non_blocking=True)
            y = y.float().cuda(non_blocking=True)
            
            optimizer.zero_grad()
            
            with torch.autocast(device_type='cuda', dtype=torch.bfloat16):
                x_out = model(x)
                loss = criterion(x_out, y)
            
            running_loss.append(loss.cpu().detach().numpy())
            
            if step % 10000 == 10000 - 1:
                print(f"epoch={epoch + 1}/{EPOCHS}-{step + 1} - loss={np.mean(running_loss)}")
                writer.add_scalar('Loss_iter/train', np.mean(running_loss[-(10000 - 1):]), epoch * len(dataloader) + step + 1)
                writer.flush()
            
            loss.backward()
            optimizer.step()
        
        torch.save({
            "model_state_dict": model.state_dict(),
            "optimzier_state_dict": optimizer.state_dict()
        }, f"chpt_{epoch + 1}.pth")
    
        print(f"epoch={epoch + 1}/{EPOCHS} - loss={np.mean(running_loss)}")
        writer.add_scalar('Loss/train', np.mean(running_loss), epoch)


def save_torchscript(model):
    
    model = model.cpu()
    model.eval()
    
    example = torch.rand(64)
    
    traced_script_module = torch.jit.trace(model, example)
    traced_script_module.save("othello_model.pt")


if __name__ == '__main__':
    main()
