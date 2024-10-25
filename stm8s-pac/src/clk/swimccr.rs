#[doc = "Register `SWIMCCR` reader"]
pub type R = crate::R<SWIMCCR_SPEC>;
#[doc = "Register `SWIMCCR` writer"]
pub type W = crate::W<SWIMCCR_SPEC>;
#[doc = "Field `SWIMCLK` reader - "]
pub type SWIMCLK_R = crate::BitReader;
#[doc = "Field `SWIMCLK` writer - "]
pub type SWIMCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swimclk(&self) -> SWIMCLK_R {
        SWIMCLK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn swimclk(&mut self) -> SWIMCLK_W<SWIMCCR_SPEC> {
        SWIMCLK_W::new(self, 0)
    }
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
#[doc = "SWIM clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swimccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swimccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIMCCR_SPEC;
impl crate::RegisterSpec for SWIMCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swimccr::R`](R) reader structure"]
impl crate::Readable for SWIMCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swimccr::W`](W) writer structure"]
impl crate::Writable for SWIMCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIMCCR to value 0"]
impl crate::Resettable for SWIMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
