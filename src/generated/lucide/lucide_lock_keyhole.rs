use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 10V7a5 5 0 0 1 10 0v3" ></ path > < / > } } pub const LucideLockKeyhole : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;