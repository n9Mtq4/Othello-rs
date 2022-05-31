import numpy as np
import pandas as pd
from numba import njit
import random
import torch
from torch.utils.data import Dataset
import othello_symmetry


@njit(nogil=True)
def board2vec(bb):
    v = np.zeros(64)
    for i in np.arange(64, dtype=np.uint64):
        v[i] = (bb >> i) & np.uint64(1)
    return v


@njit(nogil=True)
def longs2vec(me, enemy):
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
        data_path,
    ):
        self.data_path = data_path
        if self.data_path.endswith('.csv'):
            df = pd.read_csv(
                data_path,
                names=['me', 'enemy', 'score', 'moves', 'move'],
                dtype={
                    'me': np.uint64,
                    'enemy': np.uint64,
                    'score': np.float32,
                    'moves': np.int8,
                    'move': np.int8
                }
            )
            self.len = len(df)
            self.me = df['me'].values
            self.enemy = df['enemy'].values
            self.score = df['score'].values
            # self.moves = df['moves'].values
            # self.move = df['move'].values
            del df
        elif self.data_path.endswith('.npz'):
            with np.load(data_path) as data:
                self.me = data['me']
                self.enemy = data['enemy']
                self.score = data['score']
                self.len = len(self.score)
        else:
            raise RuntimeError(f'data_path must be a .csv or .npz file. Got {data_path}')
    
    def __len__(self):
        return self.len
    
    def __getitem__(self, idx):
        
        me = self.me[idx]
        enemy = self.enemy[idx]
        q = self.score[idx] / 64.0
        
        me_vec = board2vec(me)
        enemy_vec = board2vec(enemy)
        
        # sym = random.randrange(8)
        # me_vec = othello_symmetry.apply_to_board(othello_symmetry.SYMMETRIES[sym], me_vec)
        # enemy_vec = othello_symmetry.apply_to_board(othello_symmetry.SYMMETRIES[sym], enemy_vec)
        
        return torch.from_numpy(me_vec), torch.from_numpy(enemy_vec), torch.tensor([q])
