import numpy as np
import pandas as pd
from numba import njit
import random
import torch
from torch.utils.data import Dataset
import othello_symmetry


@njit(nogil=True)
def long2vec(me, enemy):
    v = np.zeros(64)
    for i in range(64):
        if me & np.uint64(1 << i) != 0:
            v[i] = 1
        elif enemy & np.uint64(1 << i) != 0:
            v[i] = -1
    return v


class OthelloNegaDataset(Dataset):
    
    def __init__(
        self,
        csv_path,
    ):
        self.csv_path = csv_path
        df = pd.read_csv(
            csv_path,
            names=['player', 'black', 'white', 'score', 'moves', 'move'],
            dtype={
                'player': np.int8,
                'black': np.uint64,
                'white': np.uint64,
                'score': np.float32,
                'moves': np.int8,
                'move': np.int8
            }
        )
        self.len = len(df)
        self.player = df['player'].values
        self.black = df['black'].values
        self.white = df['white'].values
        self.score = df['score'].values
        # self.moves = df['moves'].values
        # self.move = df['move'].values
        del df
    
    def __len__(self):
        return self.len
    
    def __getitem__(self, idx):
        
        if self.player[idx] == 0:
            me = self.black[idx]
            enemy = self.white[idx]
            q = self.score[idx] / 64.0
        else:
            me = self.white[idx]
            enemy = self.black[idx]
            q = -self.score[idx] / 64.0
        
        board_vec = long2vec(me, enemy)
        
        sym = random.randrange(8)
        board_vec = othello_symmetry.apply_to_board(othello_symmetry.SYMMETRIES[sym], board_vec)
        
        return torch.from_numpy(board_vec), torch.tensor([q])
