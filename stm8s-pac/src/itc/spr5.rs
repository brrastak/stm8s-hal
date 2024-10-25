#[doc = "Register `SPR5` reader"]
pub type R = crate::R<SPR5_SPEC>;
#[doc = "Register `SPR5` writer"]
pub type W = crate::W<SPR5_SPEC>;
#[doc = "Field `VECT16SPR` reader - "]
pub type VECT16SPR_R = crate::FieldReader;
#[doc = "Field `VECT16SPR` writer - "]
pub type VECT16SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT17SPR` reader - "]
pub type VECT17SPR_R = crate::FieldReader;
#[doc = "Field `VECT17SPR` writer - "]
pub type VECT17SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT18SPR` reader - "]
pub type VECT18SPR_R = crate::FieldReader;
#[doc = "Field `VECT18SPR` writer - "]
pub type VECT18SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT19SPR` reader - "]
pub type VECT19SPR_R = crate::FieldReader;
#[doc = "Field `VECT19SPR` writer - "]
pub type VECT19SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect16spr(&self) -> VECT16SPR_R {
        VECT16SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect17spr(&self) -> VECT17SPR_R {
        VECT17SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect18spr(&self) -> VECT18SPR_R {
        VECT18SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect19spr(&self) -> VECT19SPR_R {
        VECT19SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect16spr(&mut self) -> VECT16SPR_W<SPR5_SPEC> {
        VECT16SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect17spr(&mut self) -> VECT17SPR_W<SPR5_SPEC> {
        VECT17SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect18spr(&mut self) -> VECT18SPR_W<SPR5_SPEC> {
        VECT18SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect19spr(&mut self) -> VECT19SPR_W<SPR5_SPEC> {
        VECT19SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR5_SPEC;
impl crate::RegisterSpec for SPR5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr5::R`](R) reader structure"]
impl crate::Readable for SPR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr5::W`](W) writer structure"]
impl crate::Writable for SPR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR5 to value 0xff"]
impl crate::Resettable for SPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
