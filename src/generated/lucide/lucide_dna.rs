use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10 16 1.5 1.5" ></ path > < path d = "m14 8-1.5-1.5" ></ path > < path d = "M15 2c-1.798 1.998-2.518 3.995-2.807 5.993" ></ path > < path d = "m16.5 10.5 1 1" ></ path > < path d = "m17 6-2.891-2.891" ></ path > < path d = "M2 15c6.667-6 13.333 0 20-6" ></ path > < path d = "m20 9 .891.891" ></ path > < path d = "M3.109 14.109 4 15" ></ path > < path d = "m6.5 12.5 1 1" ></ path > < path d = "m7 18 2.891 2.891" ></ path > < path d = "M9 22c1.798-1.998 2.518-3.995 2.807-5.993" ></ path > < / > } } pub const LUCIDE_DNA : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;