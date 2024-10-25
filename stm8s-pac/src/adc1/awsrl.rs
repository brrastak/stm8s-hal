#[doc = "Register `AWSRL` reader"]
pub type R = crate::R<AWSRL_SPEC>;
#[doc = "Register `AWSRL` writer"]
pub type W = crate::W<AWSRL_SPEC>;
#[doc = "Field `AWS0` reader - "]
pub type AWS0_R = crate::BitReader;
#[doc = "Field `AWS0` writer - "]
pub type AWS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS1` reader - "]
pub type AWS1_R = crate::BitReader;
#[doc = "Field `AWS1` writer - "]
pub type AWS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS2` reader - "]
pub type AWS2_R = crate::BitReader;
#[doc = "Field `AWS2` writer - "]
pub type AWS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS3` reader - "]
pub type AWS3_R = crate::BitReader;
#[doc = "Field `AWS3` writer - "]
pub type AWS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS4` reader - "]
pub type AWS4_R = crate::BitReader;
#[doc = "Field `AWS4` writer - "]
pub type AWS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS5` reader - "]
pub type AWS5_R = crate::BitReader;
#[doc = "Field `AWS5` writer - "]
pub type AWS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS6` reader - "]
pub type AWS6_R = crate::BitReader;
#[doc = "Field `AWS6` writer - "]
pub type AWS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWS7` reader - "]
pub type AWS7_R = crate::BitReader;
#[doc = "Field `AWS7` writer - "]
pub type AWS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aws0(&self) -> AWS0_R {
        AWS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aws1(&self) -> AWS1_R {
        AWS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aws2(&self) -> AWS2_R {
        AWS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aws3(&self) -> AWS3_R {
        AWS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn aws4(&self) -> AWS4_R {
        AWS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn aws5(&self) -> AWS5_R {
        AWS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aws6(&self) -> AWS6_R {
        AWS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn aws7(&self) -> AWS7_R {
        AWS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aws0(&mut self) -> AWS0_W<AWSRL_SPEC> {
        AWS0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn aws1(&mut self) -> AWS1_W<AWSRL_SPEC> {
        AWS1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn aws2(&mut self) -> AWS2_W<AWSRL_SPEC> {
        AWS2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn aws3(&mut self) -> AWS3_W<AWSRL_SPEC> {
        AWS3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn aws4(&mut self) -> AWS4_W<AWSRL_SPEC> {
        AWS4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn aws5(&mut self) -> AWS5_W<AWSRL_SPEC> {
        AWS5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn aws6(&mut self) -> AWS6_W<AWSRL_SPEC> {
        AWS6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn aws7(&mut self) -> AWS7_W<AWSRL_SPEC> {
        AWS7_W::new(self, 7)
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
#[doc = "ADC analog watchdog status register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awsrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWSRL_SPEC;
impl crate::RegisterSpec for AWSRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awsrl::R`](R) reader structure"]
impl crate::Readable for AWSRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awsrl::W`](W) writer structure"]
impl crate::Writable for AWSRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWSRL to value 0"]
impl crate::Resettable for AWSRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
