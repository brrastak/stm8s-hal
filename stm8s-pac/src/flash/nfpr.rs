#[doc = "Register `NFPR` reader"]
pub type R = crate::R<NFPR_SPEC>;
#[doc = "Register `NFPR` writer"]
pub type W = crate::W<NFPR_SPEC>;
#[doc = "Field `NWPB0` reader - "]
pub type NWPB0_R = crate::BitReader;
#[doc = "Field `NWPB0` writer - "]
pub type NWPB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPB1` reader - "]
pub type NWPB1_R = crate::BitReader;
#[doc = "Field `NWPB1` writer - "]
pub type NWPB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPB2` reader - "]
pub type NWPB2_R = crate::BitReader;
#[doc = "Field `NWPB2` writer - "]
pub type NWPB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPB3` reader - "]
pub type NWPB3_R = crate::BitReader;
#[doc = "Field `NWPB3` writer - "]
pub type NWPB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPB4` reader - "]
pub type NWPB4_R = crate::BitReader;
#[doc = "Field `NWPB4` writer - "]
pub type NWPB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPB5` reader - "]
pub type NWPB5_R = crate::BitReader;
#[doc = "Field `NWPB5` writer - "]
pub type NWPB5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nwpb0(&self) -> NWPB0_R {
        NWPB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nwpb1(&self) -> NWPB1_R {
        NWPB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nwpb2(&self) -> NWPB2_R {
        NWPB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nwpb3(&self) -> NWPB3_R {
        NWPB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nwpb4(&self) -> NWPB4_R {
        NWPB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nwpb5(&self) -> NWPB5_R {
        NWPB5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nwpb0(&mut self) -> NWPB0_W<NFPR_SPEC> {
        NWPB0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn nwpb1(&mut self) -> NWPB1_W<NFPR_SPEC> {
        NWPB1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nwpb2(&mut self) -> NWPB2_W<NFPR_SPEC> {
        NWPB2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn nwpb3(&mut self) -> NWPB3_W<NFPR_SPEC> {
        NWPB3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn nwpb4(&mut self) -> NWPB4_W<NFPR_SPEC> {
        NWPB4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn nwpb5(&mut self) -> NWPB5_W<NFPR_SPEC> {
        NWPB5_W::new(self, 5)
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
#[doc = "Flash complementary protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nfpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nfpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NFPR_SPEC;
impl crate::RegisterSpec for NFPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nfpr::R`](R) reader structure"]
impl crate::Readable for NFPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nfpr::W`](W) writer structure"]
impl crate::Writable for NFPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NFPR to value 0xff"]
impl crate::Resettable for NFPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
