use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v10" ></ path > < path d = "M18.4 6.6a9 9 0 1 1-12.77.04" ></ path > < / > } } pub const LucidePower : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;