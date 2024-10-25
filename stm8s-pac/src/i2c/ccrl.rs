#[doc = "Register `CCRL` reader"]
pub type R = crate::R<CCRL_SPEC>;
#[doc = "Register `CCRL` writer"]
pub type W = crate::W<CCRL_SPEC>;
#[doc = "Field `CCR` reader - "]
pub type CCR_R = crate::FieldReader;
#[doc = "Field `CCR` writer - "]
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CCRL_SPEC> {
        CCR_W::new(self, 0)
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
#[doc = "I2C Clock control register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRL_SPEC;
impl crate::RegisterSpec for CCRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccrl::R`](R) reader structure"]
impl crate::Readable for CCRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccrl::W`](W) writer structure"]
impl crate::Writable for CCRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCRL to value 0"]
impl crate::Resettable for CCRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
