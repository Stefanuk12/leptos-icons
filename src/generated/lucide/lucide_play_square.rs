use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" y = "3" rx = "2" width = "18" ></ rect > < path d = "m9 8 6 4-6 4Z" ></ path > < / > } } pub const LUCIDE_PLAY_SQUARE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24")] } ;