use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "4" height = "16" x = "6" y = "4" ></ rect > < rect height = "16" y = "4" x = "14" width = "4" ></ rect > < / > } } pub const LucidePause : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2")] } ;