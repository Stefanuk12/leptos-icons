use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" height = "18" width = "18" x = "3" ></ rect > < path d = "m9 8 6 4-6 4Z" ></ path > < / > } } pub const LUCIDE_SQUARE_PLAY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;