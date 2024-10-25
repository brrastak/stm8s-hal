#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `C10` reader - "]
pub type C10_R = crate::BitReader;
#[doc = "Field `C10` writer - "]
pub type C10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C11` reader - "]
pub type C11_R = crate::BitReader;
#[doc = "Field `C11` writer - "]
pub type C11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C12` reader - "]
pub type C12_R = crate::BitReader;
#[doc = "Field `C12` writer - "]
pub type C12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C13` reader - "]
pub type C13_R = crate::BitReader;
#[doc = "Field `C13` writer - "]
pub type C13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C14` reader - "]
pub type C14_R = crate::BitReader;
#[doc = "Field `C14` writer - "]
pub type C14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C15` reader - "]
pub type C15_R = crate::BitReader;
#[doc = "Field `C15` writer - "]
pub type C15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C16` reader - "]
pub type C16_R = crate::BitReader;
#[doc = "Field `C16` writer - "]
pub type C16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C17` reader - "]
pub type C17_R = crate::BitReader;
#[doc = "Field `C17` writer - "]
pub type C17_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c10(&self) -> C10_R {
        C10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c11(&self) -> C11_R {
        C11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c12(&self) -> C12_R {
        C12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c13(&self) -> C13_R {
        C13_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn c14(&self) -> C14_R {
        C14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn c15(&self) -> C15_R {
        C15_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn c16(&self) -> C16_R {
        C16_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn c17(&self) -> C17_R {
        C17_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn c10(&mut self) -> C10_W<CR1_SPEC> {
        C10_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn c11(&mut self) -> C11_W<CR1_SPEC> {
        C11_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn c12(&mut self) -> C12_W<CR1_SPEC> {
        C12_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn c13(&mut self) -> C13_W<CR1_SPEC> {
        C13_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn c14(&mut self) -> C14_W<CR1_SPEC> {
        C14_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn c15(&mut self) -> C15_W<CR1_SPEC> {
        C15_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn c16(&mut self) -> C16_W<CR1_SPEC> {
        C16_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn c17(&mut self) -> C17_W<CR1_SPEC> {
        C17_W::new(self, 7)
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
#[doc = "Port F control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
