use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" rx = "2" y = "3" ></ rect > < path d = "m8 14 4-4 4 4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24")] } ;