#[doc = "Register `OISR` reader"]
pub type R = crate::R<OISR_SPEC>;
#[doc = "Register `OISR` writer"]
pub type W = crate::W<OISR_SPEC>;
#[doc = "Field `OIS1` reader - "]
pub type OIS1_R = crate::BitReader;
#[doc = "Field `OIS1` writer - "]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - "]
pub type OIS1N_R = crate::BitReader;
#[doc = "Field `OIS1N` writer - "]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - "]
pub type OIS2_R = crate::BitReader;
#[doc = "Field `OIS2` writer - "]
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - "]
pub type OIS2N_R = crate::BitReader;
#[doc = "Field `OIS2N` writer - "]
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - "]
pub type OIS3_R = crate::BitReader;
#[doc = "Field `OIS3` writer - "]
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - "]
pub type OIS3N_R = crate::BitReader;
#[doc = "Field `OIS3N` writer - "]
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - "]
pub type OIS4_R = crate::BitReader;
#[doc = "Field `OIS4` writer - "]
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<OISR_SPEC> {
        OIS1_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<OISR_SPEC> {
        OIS1N_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<OISR_SPEC> {
        OIS2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<OISR_SPEC> {
        OIS2N_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<OISR_SPEC> {
        OIS3_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<OISR_SPEC> {
        OIS3N_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<OISR_SPEC> {
        OIS4_W::new(self, 6)
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
#[doc = "TIM1 output idle state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OISR_SPEC;
impl crate::RegisterSpec for OISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oisr::R`](R) reader structure"]
impl crate::Readable for OISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oisr::W`](W) writer structure"]
impl crate::Writable for OISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OISR to value 0"]
impl crate::Resettable for OISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
