use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 3v18" ></ path > < path d = "m14 9 3 3-3 3" ></ path > < / > } } pub const LucidePanelLeftOpen : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;