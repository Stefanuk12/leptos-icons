use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 21a8 8 0 0 0-16 0" ></ path > < / > } } pub const LucideUserRound : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;