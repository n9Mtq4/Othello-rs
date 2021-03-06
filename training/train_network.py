import os
os.environ["CUDA_VISIBLE_DEVICES"] = "1"

import numpy as np
import torch
import torch.nn as nn
from ranger21.ranger21 import Ranger21
from tqdm import tqdm

from othello_dataset import OthelloNegaDataset
from torch.utils.data import DataLoader


# LR = 0.0002
LR = 2e-5
EPOCHS = 50
NUM_WORKERS = 16
CSV_PATH = '~/workspace/OthelloGui/train_data/novello6.3M_av_ns.csv'


class ResBlock(nn.Module):
    
    def __init__(self, f):
        super().__init__()
        self.l1 = nn.Linear(f, f)
        self.a1 = nn.ReLU()
        self.l2 = nn.Linear(f, f)
        self.a2 = nn.ReLU()
    
    def forward(self, x):
        resid = x
        x = self.l1(x)
        x = self.a1(x)
        x = self.l2(x)
        x = self.a2(x + resid)
        return x


class OthelloNNet(nn.Module):
    
    def __init__(self, n):
        super().__init__()
        self.seq = nn.Sequential(
            nn.Linear(64, n),
            nn.ReLU(),
            ResBlock(n),
            ResBlock(n),
            ResBlock(n),
            ResBlock(n),
            ResBlock(n),
            ResBlock(n),
            nn.Linear(n, n // 4),
            nn.ReLU(),
            nn.Linear(n // 4, 1),
            nn.Tanh(),
        )
    
    def forward(self, x):
        x = self.seq(x)
        return x


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
    dataloader = DataLoader(dataset, batch_size=8192 // 2, shuffle=True, num_workers=NUM_WORKERS)
    
    model = OthelloNNet(512)
    
    model.load_state_dict(torch.load("old_models/model_5.pth")['model_state_dict'])
    print("loaded model chpt")
    
    model = model.cuda()
    
    print(model)
    print(f"Total params = {sum(p.numel() for p in model.parameters())}")
    print(f"Trainable params = {sum(p.numel() for p in model.parameters() if p.requires_grad)}")
    
    print("Starting training...")
    train(model, dataloader)
    
    print("Saving torchscript model...")
    save_torchscript(model)


def train(model, dataloader):
    
    criterion = ScaledMSELoss()
    
    optimizer = Ranger21(
        model.parameters(),
        lr=LR,
        num_epochs=EPOCHS,
        warmdown_start_pct=0.35,
        warmdown_min_lr=1e-5 / 4,
        num_batches_per_epoch=len(dataloader)
    )
    
    model.train()
    
    for epoch in range(EPOCHS):
        
        running_loss = []
        
        for step, (x, y) in tqdm(enumerate(dataloader), total=len(dataloader)):
            
            x = x.cuda()
            y = y.cuda()
            
            optimizer.zero_grad()
            
            x_out = model(x.float())
            loss = criterion(x_out, y.float())
            
            running_loss.append(loss.cpu().detach().numpy())
            
            loss.backward()
            optimizer.step()
        
        torch.save({
            "model_state_dict": model.state_dict(),
            "optimzier_state_dict": optimizer.state_dict()
        }, f"chpt_{epoch + 1}.pth")
    
        print(f"epoch={epoch + 1}/{EPOCHS} - loss={np.mean(running_loss)}")


def save_torchscript(model):
    
    model = model.cpu()
    model.eval()
    
    example = torch.rand(64)
    
    traced_script_module = torch.jit.trace(model, example)
    traced_script_module.save("othello_model.pt")


if __name__ == '__main__':
    main()
