#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `C` reader - "]
pub type C_R = crate::BitReader;
#[doc = "Field `C` writer - "]
pub type C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Z` reader - "]
pub type Z_R = crate::BitReader;
#[doc = "Field `Z` writer - "]
pub type Z_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NF` reader - "]
pub type NF_R = crate::BitReader;
#[doc = "Field `NF` writer - "]
pub type NF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I0` reader - "]
pub type I0_R = crate::BitReader;
#[doc = "Field `I0` writer - "]
pub type I0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H` reader - "]
pub type H_R = crate::BitReader;
#[doc = "Field `H` writer - "]
pub type H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I1` reader - "]
pub type I1_R = crate::BitReader;
#[doc = "Field `I1` writer - "]
pub type I1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V` reader - "]
pub type V_R = crate::BitReader;
#[doc = "Field `V` writer - "]
pub type V_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i0(&self) -> I0_R {
        I0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i1(&self) -> I1_R {
        I1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> C_W<CCR_SPEC> {
        C_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn z(&mut self) -> Z_W<CCR_SPEC> {
        Z_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NF_W<CCR_SPEC> {
        NF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn i0(&mut self) -> I0_W<CCR_SPEC> {
        I0_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h(&mut self) -> H_W<CCR_SPEC> {
        H_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn i1(&mut self) -> I1_W<CCR_SPEC> {
        I1_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn v(&mut self) -> V_W<CCR_SPEC> {
        V_W::new(self, 7)
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
#[doc = "Condition code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x28"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
