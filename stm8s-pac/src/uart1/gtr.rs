#[doc = "Register `GTR` reader"]
pub type R = crate::R<GTR_SPEC>;
#[doc = "Register `GTR` writer"]
pub type W = crate::W<GTR_SPEC>;
#[doc = "Field `GT` reader - "]
pub type GT_R = crate::FieldReader;
#[doc = "Field `GT` writer - "]
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<GTR_SPEC> {
        GT_W::new(self, 0)
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
#[doc = "UART1 guard time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTR_SPEC;
impl crate::RegisterSpec for GTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gtr::R`](R) reader structure"]
impl crate::Readable for GTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtr::W`](W) writer structure"]
impl crate::Writable for GTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTR to value 0"]
impl crate::Resettable for GTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
