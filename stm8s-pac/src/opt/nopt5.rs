#[doc = "Register `NOPT5` reader"]
pub type R = crate::R<NOPT5_SPEC>;
#[doc = "Register `NOPT5` writer"]
pub type W = crate::W<NOPT5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<NOPT5_SPEC> {
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
#[doc = "HSE clock startup (complementary byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nopt5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nopt5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NOPT5_SPEC;
impl crate::RegisterSpec for NOPT5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nopt5::R`](R) reader structure"]
impl crate::Readable for NOPT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nopt5::W`](W) writer structure"]
impl crate::Writable for NOPT5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOPT5 to value 0xff"]
impl crate::Resettable for NOPT5_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
