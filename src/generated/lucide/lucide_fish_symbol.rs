use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 16s9-15 20-4C11 23 2 8 2 8" ></ path > < / > } } pub const LUCIDE_FISH_SYMBOL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;