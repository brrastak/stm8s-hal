#[doc = "Register `AWSRH` reader"]
pub type R = crate::R<AWSRH_SPEC>;
#[doc = "Register `AWSRH` writer"]
pub type W = crate::W<AWSRH_SPEC>;
#[doc = "Field `AWS8` reader - "]
pub type AWS8_R = crate::BitReader;
#[doc = "Field `AWS8` writer - "]
pub type AWS8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS9` reader - "]
pub type AWS9_R = crate::BitReader;
#[doc = "Field `AWS9` writer - "]
pub type AWS9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aws8(&self) -> AWS8_R {
        AWS8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aws9(&self) -> AWS9_R {
        AWS9_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aws8(&mut self) -> AWS8_W<AWSRH_SPEC> {
        AWS8_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn aws9(&mut self) -> AWS9_W<AWSRH_SPEC> {
        AWS9_W::new(self, 1)
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
#[doc = "ADC analog watchdog status register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awsrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWSRH_SPEC;
impl crate::RegisterSpec for AWSRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awsrh::R`](R) reader structure"]
impl crate::Readable for AWSRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awsrh::W`](W) writer structure"]
impl crate::Writable for AWSRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWSRH to value 0"]
impl crate::Resettable for AWSRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
