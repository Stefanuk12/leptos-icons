use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3" ></ path > < path d = "M2 16a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v1.5a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5V11a2 2 0 0 0-4 0z" ></ path > < path d = "M4 18v2" ></ path > < path d = "M20 18v2" ></ path > < path d = "M12 4v9" ></ path > < / > } } pub const LUCIDE_SOFA : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;