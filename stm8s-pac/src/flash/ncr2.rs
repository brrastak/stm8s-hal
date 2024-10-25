#[doc = "Register `NCR2` reader"]
pub type R = crate::R<NCR2_SPEC>;
#[doc = "Register `NCR2` writer"]
pub type W = crate::W<NCR2_SPEC>;
#[doc = "Field `NPRG` reader - "]
pub type NPRG_R = crate::BitReader;
#[doc = "Field `NPRG` writer - "]
pub type NPRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFPRG` reader - "]
pub type NFPRG_R = crate::BitReader;
#[doc = "Field `NFPRG` writer - "]
pub type NFPRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NERASE` reader - "]
pub type NERASE_R = crate::BitReader;
#[doc = "Field `NERASE` writer - "]
pub type NERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPRG` reader - "]
pub type NWPRG_R = crate::BitReader;
#[doc = "Field `NWPRG` writer - "]
pub type NWPRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOPT` reader - "]
pub type NOPT_R = crate::BitReader;
#[doc = "Field `NOPT` writer - "]
pub type NOPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nprg(&self) -> NPRG_R {
        NPRG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nfprg(&self) -> NFPRG_R {
        NFPRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nerase(&self) -> NERASE_R {
        NERASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn nwprg(&self) -> NWPRG_R {
        NWPRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn nopt(&self) -> NOPT_R {
        NOPT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nprg(&mut self) -> NPRG_W<NCR2_SPEC> {
        NPRG_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn nfprg(&mut self) -> NFPRG_W<NCR2_SPEC> {
        NFPRG_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn nerase(&mut self) -> NERASE_W<NCR2_SPEC> {
        NERASE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn nwprg(&mut self) -> NWPRG_W<NCR2_SPEC> {
        NWPRG_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn nopt(&mut self) -> NOPT_W<NCR2_SPEC> {
        NOPT_W::new(self, 7)
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
#[doc = "Flash complementary control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCR2_SPEC;
impl crate::RegisterSpec for NCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ncr2::R`](R) reader structure"]
impl crate::Readable for NCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ncr2::W`](W) writer structure"]
impl crate::Writable for NCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCR2 to value 0xff"]
impl crate::Resettable for NCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
