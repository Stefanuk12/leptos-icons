use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 19H5V13" ></ path > < path d = "M19 5L5 19" ></ path > < / > } } pub const LucideMoveDownLeft : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;