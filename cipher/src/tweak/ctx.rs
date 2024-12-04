use crypto_common::{array::ArraySize, Block, BlockSizeUser, BlockSizes};
use inout::InOut;

use super::{
    Tweak, TweakBlockCipherDecBackend, TweakBlockCipherDecClosure, TweakBlockCipherEncBackend,
    TweakBlockCipherEncClosure, TweakSizeUser,
};

/// Closure used in methods which operate over separate blocks.
pub(super) struct BlockCtx<'a, TS: ArraySize, BS: BlockSizes> {
    pub tweak: &'a Tweak<Self>,
    pub block: InOut<'a, 'a, Block<Self>>,
}

impl<TS: ArraySize, BS: BlockSizes> BlockSizeUser for BlockCtx<'_, TS, BS> {
    type BlockSize = BS;
}

impl<TS: ArraySize, BS: BlockSizes> TweakSizeUser for BlockCtx<'_, TS, BS> {
    type TweakSize = TS;
}

impl<TS: ArraySize, BS: BlockSizes> TweakBlockCipherEncClosure for BlockCtx<'_, TS, BS> {
    #[inline(always)]
    fn call<B: TweakBlockCipherEncBackend<BlockSize = BS, TweakSize = TS>>(self, backend: &B) {
        backend.encrypt_block(self.tweak, self.block);
    }
}

impl<TS: ArraySize, BS: BlockSizes> TweakBlockCipherDecClosure for BlockCtx<'_, TS, BS> {
    #[inline(always)]
    fn call<B: TweakBlockCipherDecBackend<BlockSize = BS, TweakSize = TS>>(self, backend: &B) {
        backend.decrypt_block(self.tweak, self.block);
    }
}
