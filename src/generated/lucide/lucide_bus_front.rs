use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 6 2 7" ></ path > < path d = "M10 6h4" ></ path > < path d = "m22 7-2-1" ></ path > < rect width = "16" y = "3" x = "4" rx = "2" height = "16" ></ rect > < path d = "M4 11h16" ></ path > < path d = "M8 15h.01" ></ path > < path d = "M16 15h.01" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 21v-2" ></ path > < / > } } pub const LucideBusFront : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;