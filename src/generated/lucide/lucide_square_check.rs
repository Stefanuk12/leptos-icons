use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" width = "18" height = "18" y = "3" ></ rect > < path d = "m9 12 2 2 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHECK : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;