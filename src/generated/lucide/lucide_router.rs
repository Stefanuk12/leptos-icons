use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" x = "2" rx = "2" width = "20" y = "14" ></ rect > < path d = "M6.01 18H6" ></ path > < path d = "M10.01 18H10" ></ path > < path d = "M15 10v4" ></ path > < path d = "M17.84 7.17a4 4 0 0 0-5.66 0" ></ path > < path d = "M20.66 4.34a8 8 0 0 0-11.31 0" ></ path > < / > } } pub const LUCIDE_ROUTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;