#[doc = "Register `PSCR` reader"]
pub type R = crate::R<PSCR_SPEC>;
#[doc = "Register `PSCR` writer"]
pub type W = crate::W<PSCR_SPEC>;
#[doc = "Field `PSC` reader - "]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - "]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSCR_SPEC> {
        PSC_W::new(self, 0)
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
#[doc = "TIM2 prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCR_SPEC;
impl crate::RegisterSpec for PSCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pscr::R`](R) reader structure"]
impl crate::Readable for PSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pscr::W`](W) writer structure"]
impl crate::Writable for PSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
