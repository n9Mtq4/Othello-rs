import os
os.environ["CUDA_VISIBLE_DEVICES"] = "1"

import numpy as np
import torch
import torch.nn as nn
from pathlib import Path
from functools import reduce


class ResBlock(nn.Module):
    
    def __init__(self, f):
        super().__init__()
        self.l1 = nn.Linear(f, f)
        self.a1 = nn.LeakyReLU()
        self.l2 = nn.Linear(f, f, bias=False)
        self.bn = nn.BatchNorm1d(f)
        self.a2 = nn.LeakyReLU()
    
    def forward(self, x):
        resid = x
        x = self.l1(x)
        x = self.a1(x)
        x = self.l2(x)
        x = self.bn(x)
        x = self.a2(x + resid)
        return x


class OthelloNNet(nn.Module):
    
    def __init__(self, n, blocks):
        super().__init__()
        self.en = nn.Sequential(
            nn.Linear(128, n),
            nn.LeakyReLU(),
        )
        self.res_tower = nn.Sequential(*[ResBlock(n) for _ in range(blocks)])
        self.pred_head = nn.Linear(n, 128)
    
    def freeze_encoder(self):
        freeze = [self.en, self.res_tower]
        for module in freeze:
            for param in module.parameters():
                param.requires_grad = False
    
    def forward(self, x):
        x = self.en(x)
        x = self.res_tower(x)
        x = torch.sigmoid(self.pred_head(x))
        return torch.sum(x[:, :64] - x[:, 64:], 1) / 64.0


def avg_models_from_dir(dir_path, *model_args):
    chpts = list(Path(dir_path).glob('*.pth'))
    print(f"loading {chpts} to average")
    state_dicts = [torch.load(chpt)['model_state_dict'] for chpt in chpts]
    model = OthelloNNet(*model_args)
    model.load_state_dict(average(state_dicts))
    return model


def average(state_dicts):
    avg_state_dict = {}
    for key in state_dicts[0]:
        ws = [state_dict[key] for state_dict in state_dicts]
        w = reduce(lambda a, b: a + b, ws[1:], ws[0])
        avg_state_dict[key] = w / len(state_dicts)
    return avg_state_dict


def main():
    
    model = OthelloNNet(2048, 10)
    
    model.load_state_dict(torch.load("final_state/chpt_200.pth")['model_state_dict'])
    print("loaded model chpt")
    
    print(model)
    print(f"Total params = {sum(p.numel() for p in model.parameters())}")
    print(f"Trainable params = {sum(p.numel() for p in model.parameters() if p.requires_grad)}")
    
    print("Saving torchscript model...")
    save_torchscript(model)
    
    print("averaging many models")
    avg_model = avg_models_from_dir('final_state', 2048, 10)
    save_torchscript(avg_model, 'othello_model_avg.pt')


def save_torchscript(model, fname = 'othello_model.pt'):
    
    model = model.cpu()
    model.eval()
    
    example = torch.rand(128).unsqueeze(0)
    
    traced_script_module = torch.jit.trace(model, example)
    traced_script_module.save(fname)


if __name__ == '__main__':
    main()
