use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 16s-1.5-2-4-2-4 2-4 2" ></ path > < path d = "M7.5 8 10 9" ></ path > < path d = "m14 9 2.5-1" ></ path > < path d = "M9 10h0" ></ path > < path d = "M15 10h0" ></ path > < / > } } pub const LucideAngry : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;