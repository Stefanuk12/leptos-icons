use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 6 2 7" ></ path > < path d = "M10 6h4" ></ path > < path d = "m22 7-2-1" ></ path > < rect rx = "2" height = "16" y = "3" x = "4" width = "16" ></ rect > < path d = "M4 11h16" ></ path > < path d = "M8 15h.01" ></ path > < path d = "M16 15h.01" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 21v-2" ></ path > < / > } } pub const LUCIDE_BUS_FRONT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24")] } ;