use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" height = "18" y = "3" x = "3" ></ rect > < path d = "m9 8 6 4-6 4Z" ></ path > < / > } } pub const LUCIDE_SQUARE_PLAY : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;