use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 15h18" ></ path > < / > } } pub const LucidePanelBottom : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;