use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10.065 12.493-6.18 1.318a.934.934 0 0 1-1.108-.702l-.537-2.15a1.07 1.07 0 0 1 .691-1.265l13.504-4.44" ></ path > < path d = "m13.56 11.747 4.332-.924" ></ path > < path d = "m16 21-3.105-6.21" ></ path > < path d = "M16.485 5.94a2 2 0 0 1 1.455-2.425l1.09-.272a1 1 0 0 1 1.212.727l1.515 6.06a1 1 0 0 1-.727 1.213l-1.09.272a2 2 0 0 1-2.425-1.455z" ></ path > < path d = "m6.158 8.633 1.114 4.456" ></ path > < path d = "m8 21 3.105-6.21" ></ path > < circle cx = "12" cy = "13" r = "2" ></ circle > < / > } } pub const LUCIDE_TELESCOPE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;