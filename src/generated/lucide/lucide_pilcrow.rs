use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 4v16" ></ path > < path d = "M17 4v16" ></ path > < path d = "M19 4H9.5a4.5 4.5 0 0 0 0 9H13" ></ path > < / > } } pub const LucidePilcrow : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24")] } ;