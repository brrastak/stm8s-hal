#[doc = "Register `CCR4L` reader"]
pub type R = crate::R<CCR4L_SPEC>;
#[doc = "Register `CCR4L` writer"]
pub type W = crate::W<CCR4L_SPEC>;
#[doc = "Field `CCR4` reader - "]
pub type CCR4_R = crate::FieldReader;
#[doc = "Field `CCR4` writer - "]
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> CCR4_W<CCR4L_SPEC> {
        CCR4_W::new(self, 0)
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
#[doc = "TIM1 capture/compare register 4 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR4L_SPEC;
impl crate::RegisterSpec for CCR4L_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccr4l::R`](R) reader structure"]
impl crate::Readable for CCR4L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr4l::W`](W) writer structure"]
impl crate::Writable for CCR4L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR4L to value 0"]
impl crate::Resettable for CCR4L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
