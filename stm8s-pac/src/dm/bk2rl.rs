#[doc = "Register `BK2RL` reader"]
pub type R = crate::R<BK2RL_SPEC>;
#[doc = "Register `BK2RL` writer"]
pub type W = crate::W<BK2RL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK2RL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DM breakpoint 2 register low byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2rl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2rl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK2RL_SPEC;
impl crate::RegisterSpec for BK2RL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bk2rl::R`](R) reader structure"]
impl crate::Readable for BK2RL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk2rl::W`](W) writer structure"]
impl crate::Writable for BK2RL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK2RL to value 0xff"]
impl crate::Resettable for BK2RL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
