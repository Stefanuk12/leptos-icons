use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "14" x = "2" width = "20" height = "8" ></ rect > < path d = "M6.01 18H6" ></ path > < path d = "M10.01 18H10" ></ path > < path d = "M15 10v4" ></ path > < path d = "M17.84 7.17a4 4 0 0 0-5.66 0" ></ path > < path d = "M20.66 4.34a8 8 0 0 0-11.31 0" ></ path > < / > } } pub const LUCIDE_ROUTER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;