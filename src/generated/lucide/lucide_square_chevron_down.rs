use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" x = "3" width = "18" height = "18" ></ rect > < path d = "m16 10-4 4-4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_DOWN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;