use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "13" rx = "1" x = "3" height = "7" ></ rect > < path d = "m22 15-3-3 3-3" ></ path > < rect y = "14" height = "7" width = "13" rx = "1" x = "3" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;