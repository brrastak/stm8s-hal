#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `T0` reader - "]
pub type T0_R = crate::BitReader;
#[doc = "Field `T0` writer - "]
pub type T0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1` reader - "]
pub type T1_R = crate::BitReader;
#[doc = "Field `T1` writer - "]
pub type T1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T2` reader - "]
pub type T2_R = crate::BitReader;
#[doc = "Field `T2` writer - "]
pub type T2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T3` reader - "]
pub type T3_R = crate::BitReader;
#[doc = "Field `T3` writer - "]
pub type T3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T4` reader - "]
pub type T4_R = crate::BitReader;
#[doc = "Field `T4` writer - "]
pub type T4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T5` reader - "]
pub type T5_R = crate::BitReader;
#[doc = "Field `T5` writer - "]
pub type T5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T6` reader - "]
pub type T6_R = crate::BitReader;
#[doc = "Field `T6` writer - "]
pub type T6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGA` reader - "]
pub type WDGA_R = crate::BitReader;
#[doc = "Field `WDGA` writer - "]
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn t1(&self) -> T1_R {
        T1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn t2(&self) -> T2_R {
        T2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn t3(&self) -> T3_R {
        T3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t4(&self) -> T4_R {
        T4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn t5(&self) -> T5_R {
        T5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t6(&self) -> T6_R {
        T6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T0_W<CR_SPEC> {
        T0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn t1(&mut self) -> T1_W<CR_SPEC> {
        T1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn t2(&mut self) -> T2_W<CR_SPEC> {
        T2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn t3(&mut self) -> T3_W<CR_SPEC> {
        T3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn t4(&mut self) -> T4_W<CR_SPEC> {
        T4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn t5(&mut self) -> T5_W<CR_SPEC> {
        T5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn t6(&mut self) -> T6_W<CR_SPEC> {
        T6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<CR_SPEC> {
        WDGA_W::new(self, 7)
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
#[doc = "WWDG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
