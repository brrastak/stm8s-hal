#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `CEN` reader - "]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - "]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - "]
pub type UDIS_R = crate::BitReader;
#[doc = "Field `UDIS` writer - "]
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - "]
pub type URS_R = crate::BitReader;
#[doc = "Field `URS` writer - "]
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - "]
pub type OPM_R = crate::BitReader;
#[doc = "Field `OPM` writer - "]
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - "]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - "]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMS` reader - "]
pub type CMS_R = crate::FieldReader;
#[doc = "Field `CMS` writer - "]
pub type CMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPE` reader - "]
pub type ARPE_R = crate::BitReader;
#[doc = "Field `ARPE` writer - "]
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CR1_SPEC> {
        CEN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<CR1_SPEC> {
        UDIS_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<CR1_SPEC> {
        URS_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<CR1_SPEC> {
        OPM_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CR1_SPEC> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<CR1_SPEC> {
        CMS_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<CR1_SPEC> {
        ARPE_W::new(self, 7)
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
#[doc = "TIM1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
