use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5h0c-1.4 0-2.5-1.1-2.5-2.5V2" ></ path > < path d = "M8.5 2h7" ></ path > < path d = "M14.5 16h-5" ></ path > < / > } } pub const LUCIDE_TEST_TUBE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24")] } ;