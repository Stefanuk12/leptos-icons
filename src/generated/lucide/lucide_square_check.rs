use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" x = "3" height = "18" rx = "2" ></ rect > < path d = "m9 12 2 2 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHECK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;