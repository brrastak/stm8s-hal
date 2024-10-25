#[doc = "Register `CCR1L` reader"]
pub type R = crate::R<CCR1L_SPEC>;
#[doc = "Register `CCR1L` writer"]
pub type W = crate::W<CCR1L_SPEC>;
#[doc = "Field `CCR1` reader - "]
pub type CCR1_R = crate::FieldReader;
#[doc = "Field `CCR1` writer - "]
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<CCR1L_SPEC> {
        CCR1_W::new(self, 0)
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
#[doc = "TIM2 capture/compare register 1 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR1L_SPEC;
impl crate::RegisterSpec for CCR1L_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccr1l::R`](R) reader structure"]
impl crate::Readable for CCR1L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr1l::W`](W) writer structure"]
impl crate::Writable for CCR1L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR1L to value 0"]
impl crate::Resettable for CCR1L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
