use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 15c6.667-6 13.333 0 20-6" ></ path > < path d = "M9 22c1.798-1.998 2.518-3.995 2.807-5.993" ></ path > < path d = "M15 2c-1.798 1.998-2.518 3.995-2.807 5.993" ></ path > < path d = "m17 6-2.5-2.5" ></ path > < path d = "m14 8-1-1" ></ path > < path d = "m7 18 2.5 2.5" ></ path > < path d = "m3.5 14.5.5.5" ></ path > < path d = "m20 9 .5.5" ></ path > < path d = "m6.5 12.5 1 1" ></ path > < path d = "m16.5 10.5 1 1" ></ path > < path d = "m10 16 1.5 1.5" ></ path > < / > } } pub const LUCIDE_DNA : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;