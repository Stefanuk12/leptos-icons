use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "2" cx = "16" cy = "20" ></ circle > < path d = "M10 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v2" ></ path > < path d = "m22 14-4.5 4.5" ></ path > < path d = "m21 15 1 1" ></ path > < / > } } pub const LucideFolderKey : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none")] } ;