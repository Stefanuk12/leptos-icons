use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 9h18" ></ path > < path d = "m9 16 3-3 3 3" ></ path > < / > } } pub const LucidePanelTopClose : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;