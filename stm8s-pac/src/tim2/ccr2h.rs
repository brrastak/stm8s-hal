#[doc = "Register `CCR2H` reader"]
pub type R = crate::R<CCR2H_SPEC>;
#[doc = "Register `CCR2H` writer"]
pub type W = crate::W<CCR2H_SPEC>;
#[doc = "Field `CCR2` reader - "]
pub type CCR2_R = crate::FieldReader;
#[doc = "Field `CCR2` writer - "]
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> CCR2_W<CCR2H_SPEC> {
        CCR2_W::new(self, 0)
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
#[doc = "TIM2 capture/compare reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR2H_SPEC;
impl crate::RegisterSpec for CCR2H_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccr2h::R`](R) reader structure"]
impl crate::Readable for CCR2H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr2h::W`](W) writer structure"]
impl crate::Writable for CCR2H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR2H to value 0"]
impl crate::Resettable for CCR2H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
