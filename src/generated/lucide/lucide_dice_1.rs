use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12h.01" ></ path > < / > } } pub const LucideDice1 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;