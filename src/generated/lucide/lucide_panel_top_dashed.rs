use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 9h1" ></ path > < path d = "M19 9h2" ></ path > < path d = "M3 9h2" ></ path > < path d = "M9 9h1" ></ path > < / > } } pub const LucidePanelTopDashed : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;