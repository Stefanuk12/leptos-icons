use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" rx = "2" x = "2" width = "20" height = "16" ></ rect > < path d = "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" ></ path > < / > } } pub const LUCIDE_MAIL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;