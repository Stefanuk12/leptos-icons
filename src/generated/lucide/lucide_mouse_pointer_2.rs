use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4 4 7.07 17 2.51-7.39L21 11.07z" ></ path > < / > } } pub const LucideMousePointer2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;